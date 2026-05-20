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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1995;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1998;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta595<F: Float>(t1940: F, t2071: F, t9342: F, t28309: F, t686: F, t72: F, t25375: F, t1957: F, t28425: F, t25372: F, t98809: F, t25386: F, t95822: F, t98815: F, t95537: F, t25310: F, t28360: F, t25365: F, t26485: F, t99466: F, t28377: F, t689: F, t25431: F, t14979: F, t7403: F, t95538: F, t95542: F, t95543: F, t95548: F, t25411: F, t25387: F, t28404: F, t28384: F, t1558: F, t25391: F, t95551: F, t95553: F, t95556: F, t95562: F, t95567: F, t95569: F, t95572: F, t95576: F, t99155: F, t136: F, t2457: F, t8006: F, t93377: F, t28314: F, t93342: F, t28417: F, t2435: F, t8011: F, t2439: F, t93170: F, t28347: F, t25383: F, t2772: F, t28348: F, t28394: F, t95594: F, t95598: F, t95604: F, t95607: F, t95613: F, t95620: F, t93190: F, t10073: F, t26554: F, t27198: F, t15003: F, t95773: F, t15030: F, t26550: F, t95624: F, t95629: F, t95632: F, t95635: F, t95645: F, t95647: F, t95649: F, t95651: F, t99309: F, t99369: F, t1579: F, t2718: F, t7398: F, t26506: F, t27216: F, t14587: F, t25317: F, t25394: F, t27349: F, t27353: F, t2828: F, t28426: F, t28439: F, t28442: F, t7070: F, t92917: F, t93349: F, t95720: F, t95722: F, t95727: F, t95732: F, t95733: F, t95825: F, t99237: F, t786: F, t7998: F, t867: F, t2467: F, t1580: F, t26446: F, t28368: F, t93321: F, t93374: F, t26511: F, t26551: F, t26568: F, t26573: F, t27199: F, t2771: F, t28400: F, t51698: F, t7067: F, t7997: F, t95740: F, t95744: F, t95747: F, t99191: F, t99277: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t102917, t102928, t102930, t102934, t102937) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1995::<F>(t1940, t2071, t9342, t28309, t686, t72, t25375, t1957, t28425, t25372, t98809, t25386);
        let (t102951, t102954) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1996::<F>(t95822, t98815, t95537, t25310, t28360, t25365, t26485, t99466, t28377, t689, t25431, t102930, t102934, t102937, t14979, t7403, t95538, t95542, t95543, t95548);
        let (t102972, t102977) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1997::<F>(t102951, t25411, t102928, t25387, t28404, t689, t25431, t28384, t1558, t25391, t28425, t95551, t95553, t95556, t95562, t95567, t95569, t95572, t95576, t99155);
        let (t102980, t102981, t102984, t102986, t102988, t102993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1998::<F>(t136, t2457, t8006, t93377, t28314, t93342, t28417, t686, t72, t25375, t2435, t8011);
        let (t103000, t103005, t103008) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999::<F>(t102993, t25431, t2439, t8011, t93170, t28347, t686, t72, t25387, t102981, t102984, t102988, t25383, t2772, t28348, t28394, t95594, t95598, t95604, t95607, t95613, t95620);
        let t103033 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2000::<F>(t102980, t93190, t10073, t26554, t27198, t102972, t25411, t15003, t95773, t15030, t25391, t26550, t28425, t7403, t95624, t95629, t95632, t95635, t95645, t95647, t95649, t95651, t99309, t99369);
        let t103065 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001::<F>(t1579, t26550, t103005, t25375, t2718, t7398, t26506, t27216, t14587, t25317, t25391, t25394, t27349, t27353, t2828, t28426, t28439, t28442, t7070, t8006, t92917, t93349, t95720, t95722, t95727, t95732, t95733, t95825, t99237);
        let t103100 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002::<F>(t786, t7998, t867, t2467, t1580, t26446, t689, t28368, t93321, t93374, t25317, t26511, t26550, t26551, t26568, t26573, t27199, t27353, t2771, t28400, t51698, t7067, t7070, t7997, t93349, t95740, t95744, t95747, t99191, t99277);
    (t102917, t102954, t102977, t102986, t102993, t103000, t103008, t103033, t103065, t103100)
}
