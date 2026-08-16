//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2009;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2010;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2011;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2012;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2013;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2014;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2015;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2016;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2017;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2018;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta628(t30410: f64, t686: f64, t72: f64, t93317: f64, t102971: f64, t102974: f64, t102981: f64, t102984: f64, t102988: f64, t102994: f64, t103452: f64, t27353: f64, t28394: f64, t28425: f64, t4487: f64, t62589: f64, t62593: f64, t62628: f64, t95567: f64, t95569: f64, t95576: f64, t30400: f64, t689: f64, t25431: f64, t25411: f64, t103001: f64, t103007: f64, t103009: f64, t103017: f64, t103023: f64, t18615: f64, t2061: f64, t231: f64, t25383: f64, t28436: f64, t30406: f64, t4423: f64, t7070: f64, t7076: f64, t7997: f64, t95607: f64, t95620: f64, t95629: f64, t95632: f64, t99191: f64, t105946: f64, t7407: f64, t106387: f64, t30356: f64, t25387: f64, t103030: f64, t103047: f64, t103063: f64, t103069: f64, t103072: f64, t27199: f64, t28348: f64, t6016: f64, t7398: f64, t8007: f64, t95722: f64, t95727: f64, t95732: f64, t99303: f64, t30380: f64, t7058: f64, t28314: f64, t99466: f64, t7064: f64, t103086: f64, t103088: f64, t103103: f64, t103114: f64, t103119: f64, t103122: f64, t103130: f64, t103136: f64, t28310: f64, t30411: f64, t95740: f64, t95747: f64, t103067: f64, t4481: f64, t103140: f64, t103142: f64, t103156: f64, t103158: f64, t103161: f64, t106275: f64, t26550: f64, t62695: f64, t7415: f64, t95774: f64, t95779: f64, t95783: f64, t95786: f64, t95794: f64, t95796: f64, t103247: f64, t103254: f64, t105985: f64, t105987: f64, t105989: f64, t105991: f64, t105993: f64, t105995: f64, t105997: f64, t105999: f64, t106001: f64, t106003: f64, t103265: f64, t103267: f64, t106006: f64, t106008: f64, t106010: f64, t106012: f64, t106014: f64, t95666: f64, t98960: f64, t98961: f64, t98962: f64, t98964: f64, t103273: f64, t103276: f64, t103280: f64, t103283: f64, t106022: f64, t106024: f64, t95671: f64, t98976: f64, t98979: f64, t99002: f64, t99004: f64, t99009: f64, t103286: f64, t106030: f64, t106033: f64, t106035: f64, t106037: f64, t106040: f64, t106042: f64, t106044: f64, t106046: f64, t106048: f64, t106050: f64, t106053: f64, t99013: f64, t103290: f64, t103291: f64, t103292: f64, t103293: f64, t103294: f64, t103296: f64, t103301: f64, t106058: f64, t106061: f64, t106063: f64, t106065: f64, t99035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t110275, t110281) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2009(t30410, t686, t72, t93317, t102971, t102974, t102981, t102984, t102988, t102994, t103452, t27353, t28394, t28425, t4487, t62589, t62593, t62628, t95567, t95569, t95576);
        let t110306 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2010(t30400, t689, t25431, t25411, t103001, t103007, t103009, t103017, t103023, t18615, t2061, t231, t25383, t28436, t30406, t4423, t7070, t7076, t7997, t95607, t95620, t95629, t95632, t99191);
        let (t110322, t110330) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2011(t105946, t7407, t106387, t30356, t686, t72, t25387, t103030, t103047, t103063, t103069, t103072, t231, t27199, t28348, t6016, t7070, t7076, t7398, t8007, t95722, t95727, t95732, t99303);
        let t110348 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2012(t30380, t686, t72, t7058, t28314, t99466, t7064, t103086, t103088, t103103, t103114, t103119, t103122, t103130, t103136, t25383, t27199, t28310, t30411, t95740, t95747);
        let t110365 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2013(t103067, t4481, t103140, t103142, t103156, t103158, t103161, t106275, t26550, t27353, t62695, t7415, t95774, t95779, t95783, t95786, t95794, t95796);
        let t110378 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2014(t103247, t103254, t105985, t105987, t105989, t105991, t105993, t105995, t105997, t105999, t106001, t106003);
        let t110385 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2015(t103265, t103267, t106006, t106008, t106010, t106012, t106014, t95666, t98960, t98961, t98962, t98964);
        let t110393 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2016(t103273, t103276, t103280, t103283, t106022, t106024, t95671, t98976, t98979, t99002, t99004, t99009);
        let t110406 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2017(t103286, t106030, t106033, t106035, t106037, t106040, t106042, t106044, t106046, t106048, t106050, t106053, t99013);
        let t110414 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2018(t103290, t103291, t103292, t103293, t103294, t103296, t103301, t106058, t106061, t106063, t106065, t99035);
    (t110275, t110281, t110306, t110322, t110330, t110348, t110365, t110378, t110385, t110393, t110406, t110414)
}
