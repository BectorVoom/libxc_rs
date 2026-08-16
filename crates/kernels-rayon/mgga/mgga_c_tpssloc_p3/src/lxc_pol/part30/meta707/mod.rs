//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2333;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta707(t100822: f64, t100864: f64, t96749: f64, t96793: f64, t96840: f64, t97814: f64, t97859: f64, t97906: f64, t16524: f64, t26545: f64, t1873: f64, t66958: f64, t55388: f64, t7015: f64, t20173: f64, t28896: f64, t28893: f64, t6534: f64, t1401: f64, t96729: f64, t26542: f64, t1458: f64, t26135: f64, t3941: f64, t4072: f64, t7467: f64, t28017: f64, t3938: f64, t12524: f64, t28899: f64, t20176: f64, t23877: f64, t23880: f64, t26523: f64, t5456: f64, t5493: f64, t577: f64, t83980: f64, t96351: f64, t75795: f64, t7769: f64, t5371: f64, t112: f64, t28868: f64, t26550: f64, t55353: f64, t16521: f64, t19534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100867, t100871, t100873) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332(t100822, t100864, t96749, t96793, t96840, t97814, t97859, t97906, t16524, t26545, t1873, t66958);
        let (t100875, t100879, t100883, t100885, t100887, t100890) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2333(t55388, t7015, t20173, t28896, t28893, t6534, t1401, t96729, t16524, t26542, t1458, t26135, t3941);
        let t100900 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334(t3941, t4072, t7467, t28017, t3938, t12524, t28899, t100867, t100871, t100873, t100875, t100879, t100883, t100885, t100887, t100890, t20176, t23877, t23880, t26523, t5456, t5493, t577, t83980, t96351);
        let (t100902, t100908, t100911, t100915, t100917, t100921, t100924) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335(t75795, t7769, t26135, t5371, t112, t28868, t16524, t26550, t55353, t16521, t7467, t1873, t19534, t3941);
    (t100867, t100900, t100902, t100908, t100911, t100915, t100917, t100921, t100924)
}
