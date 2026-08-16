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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta628<F: Float>(t30410: F, t686: F, t72: F, t93317: F, t102971: F, t102974: F, t102981: F, t102984: F, t102988: F, t102994: F, t103452: F, t27353: F, t28394: F, t28425: F, t4487: F, t62589: F, t62593: F, t62628: F, t95567: F, t95569: F, t95576: F, t30400: F, t689: F, t25431: F, t25411: F, t103001: F, t103007: F, t103009: F, t103017: F, t103023: F, t18615: F, t2061: F, t231: F, t25383: F, t28436: F, t30406: F, t4423: F, t7070: F, t7076: F, t7997: F, t95607: F, t95620: F, t95629: F, t95632: F, t99191: F, t105946: F, t7407: F, t106387: F, t30356: F, t25387: F, t103030: F, t103047: F, t103063: F, t103069: F, t103072: F, t27199: F, t28348: F, t6016: F, t7398: F, t8007: F, t95722: F, t95727: F, t95732: F, t99303: F, t30380: F, t7058: F, t28314: F, t99466: F, t7064: F, t103086: F, t103088: F, t103103: F, t103114: F, t103119: F, t103122: F, t103130: F, t103136: F, t28310: F, t30411: F, t95740: F, t95747: F, t103067: F, t4481: F, t103140: F, t103142: F, t103156: F, t103158: F, t103161: F, t106275: F, t26550: F, t62695: F, t7415: F, t95774: F, t95779: F, t95783: F, t95786: F, t95794: F, t95796: F, t103247: F, t103254: F, t105985: F, t105987: F, t105989: F, t105991: F, t105993: F, t105995: F, t105997: F, t105999: F, t106001: F, t106003: F, t103265: F, t103267: F, t106006: F, t106008: F, t106010: F, t106012: F, t106014: F, t95666: F, t98960: F, t98961: F, t98962: F, t98964: F, t103273: F, t103276: F, t103280: F, t103283: F, t106022: F, t106024: F, t95671: F, t98976: F, t98979: F, t99002: F, t99004: F, t99009: F, t103286: F, t106030: F, t106033: F, t106035: F, t106037: F, t106040: F, t106042: F, t106044: F, t106046: F, t106048: F, t106050: F, t106053: F, t99013: F, t103290: F, t103291: F, t103292: F, t103293: F, t103294: F, t103296: F, t103301: F, t106058: F, t106061: F, t106063: F, t106065: F, t99035: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t110275, t110281) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2009::<F>(t30410, t686, t72, t93317, t102971, t102974, t102981, t102984, t102988, t102994, t103452, t27353, t28394, t28425, t4487, t62589, t62593, t62628, t95567, t95569, t95576);
        let t110306 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2010::<F>(t30400, t689, t25431, t25411, t103001, t103007, t103009, t103017, t103023, t18615, t2061, t231, t25383, t28436, t30406, t4423, t7070, t7076, t7997, t95607, t95620, t95629, t95632, t99191);
        let (t110322, t110330) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2011::<F>(t105946, t7407, t106387, t30356, t686, t72, t25387, t103030, t103047, t103063, t103069, t103072, t231, t27199, t28348, t6016, t7070, t7076, t7398, t8007, t95722, t95727, t95732, t99303);
        let t110348 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2012::<F>(t30380, t686, t72, t7058, t28314, t99466, t7064, t103086, t103088, t103103, t103114, t103119, t103122, t103130, t103136, t25383, t27199, t28310, t30411, t95740, t95747);
        let t110365 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2013::<F>(t103067, t4481, t103140, t103142, t103156, t103158, t103161, t106275, t26550, t27353, t62695, t7415, t95774, t95779, t95783, t95786, t95794, t95796);
        let t110378 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2014::<F>(t103247, t103254, t105985, t105987, t105989, t105991, t105993, t105995, t105997, t105999, t106001, t106003);
        let t110385 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2015::<F>(t103265, t103267, t106006, t106008, t106010, t106012, t106014, t95666, t98960, t98961, t98962, t98964);
        let t110393 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2016::<F>(t103273, t103276, t103280, t103283, t106022, t106024, t95671, t98976, t98979, t99002, t99004, t99009);
        let t110406 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2017::<F>(t103286, t106030, t106033, t106035, t106037, t106040, t106042, t106044, t106046, t106048, t106050, t106053, t99013);
        let t110414 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2018::<F>(t103290, t103291, t103292, t103293, t103294, t103296, t103301, t106058, t106061, t106063, t106065, t99035);
    (t110275, t110281, t110306, t110322, t110330, t110348, t110365, t110378, t110385, t110393, t110406, t110414)
}
