//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1995;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1998;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta595(t1940: f64, t2071: f64, t9342: f64, t28309: f64, t686: f64, t72: f64, t25375: f64, t1957: f64, t28425: f64, t25372: f64, t98809: f64, t25386: f64, t95822: f64, t98815: f64, t95537: f64, t25310: f64, t28360: f64, t25365: f64, t26485: f64, t99466: f64, t28377: f64, t689: f64, t25431: f64, t14979: f64, t7403: f64, t95538: f64, t95542: f64, t95543: f64, t95548: f64, t25411: f64, t25387: f64, t28404: f64, t28384: f64, t1558: f64, t25391: f64, t95551: f64, t95553: f64, t95556: f64, t95562: f64, t95567: f64, t95569: f64, t95572: f64, t95576: f64, t99155: f64, t136: f64, t2457: f64, t8006: f64, t93377: f64, t28314: f64, t93342: f64, t28417: f64, t2435: f64, t8011: f64, t2439: f64, t93170: f64, t28347: f64, t25383: f64, t2772: f64, t28348: f64, t28394: f64, t95594: f64, t95598: f64, t95604: f64, t95607: f64, t95613: f64, t95620: f64, t93190: f64, t10073: f64, t26554: f64, t27198: f64, t15003: f64, t95773: f64, t15030: f64, t26550: f64, t95624: f64, t95629: f64, t95632: f64, t95635: f64, t95645: f64, t95647: f64, t95649: f64, t95651: f64, t99309: f64, t99369: f64, t1579: f64, t2718: f64, t7398: f64, t26506: f64, t27216: f64, t14587: f64, t25317: f64, t25394: f64, t27349: f64, t27353: f64, t2828: f64, t28426: f64, t28439: f64, t28442: f64, t7070: f64, t92917: f64, t93349: f64, t95720: f64, t95722: f64, t95727: f64, t95732: f64, t95733: f64, t95825: f64, t99237: f64, t786: f64, t7998: f64, t867: f64, t2467: f64, t1580: f64, t26446: f64, t28368: f64, t93321: f64, t93374: f64, t26511: f64, t26551: f64, t26568: f64, t26573: f64, t27199: f64, t2771: f64, t28400: f64, t51698: f64, t7067: f64, t7997: f64, t95740: f64, t95744: f64, t95747: f64, t99191: f64, t99277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102917, t102928, t102930, t102934, t102937) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1995(t1940, t2071, t9342, t28309, t686, t72, t25375, t1957, t28425, t25372, t98809, t25386);
        let (t102951, t102954) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996(t95822, t98815, t95537, t25310, t28360, t25365, t26485, t99466, t28377, t689, t25431, t102930, t102934, t102937, t14979, t7403, t95538, t95542, t95543, t95548);
        let (t102972, t102977) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997(t102951, t25411, t102928, t25387, t28404, t689, t25431, t28384, t1558, t25391, t28425, t95551, t95553, t95556, t95562, t95567, t95569, t95572, t95576, t99155);
        let (t102980, t102981, t102984, t102986, t102988, t102993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1998(t136, t2457, t8006, t93377, t28314, t93342, t28417, t686, t72, t25375, t2435, t8011);
        let (t103000, t103005, t103008) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999(t102993, t25431, t2439, t8011, t93170, t28347, t686, t72, t25387, t102981, t102984, t102988, t25383, t2772, t28348, t28394, t95594, t95598, t95604, t95607, t95613, t95620);
        let t103033 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000(t102980, t93190, t10073, t26554, t27198, t102972, t25411, t15003, t95773, t15030, t25391, t26550, t28425, t7403, t95624, t95629, t95632, t95635, t95645, t95647, t95649, t95651, t99309, t99369);
        let t103065 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001(t1579, t26550, t103005, t25375, t2718, t7398, t26506, t27216, t14587, t25317, t25391, t25394, t27349, t27353, t2828, t28426, t28439, t28442, t7070, t8006, t92917, t93349, t95720, t95722, t95727, t95732, t95733, t95825, t99237);
        let t103100 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002(t786, t7998, t867, t2467, t1580, t26446, t689, t28368, t93321, t93374, t25317, t26511, t26550, t26551, t26568, t26573, t27199, t27353, t2771, t28400, t51698, t7067, t7070, t7997, t93349, t95740, t95744, t95747, t99191, t99277);
    (t102917, t102954, t102977, t102986, t102993, t103000, t103008, t103033, t103065, t103100)
}
