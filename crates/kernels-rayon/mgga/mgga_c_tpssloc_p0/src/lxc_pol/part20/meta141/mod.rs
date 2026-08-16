//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk912;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk913;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk914;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk915;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk916;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk917;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk918;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta141(t3236: f64, t3293: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3288: f64, t3290: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64, t1156: f64, t1146: f64, t440: f64, t448: f64, t3377: f64, t1129: f64, t1138: f64, t1148: f64, t1157: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t3324: f64, t3327: f64, t3332: f64, t3334: f64, t3352: f64, t3357: f64, t3360: f64, t3369: f64, t3371: f64, t3376: f64, t3378: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3383, t3390, t3395) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk912(t3236, t3293, t3238, t3245, t3250, t3254, t3272, t3280, t3288, t3290, t3295, t3299, t3302, t3305);
        let t3396 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk913(t1156, t3395);
        let t3399 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk914(t1146);
        let t3400 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk915(t3399);
        let t3401 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk916(t3400, t440);
        let (t3402, t3403) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk917(t448);
        let t3404 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk918(t3377, t3403);
        let t3407 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk919(t1129, t1138, t1148, t1157, t3258, t3261, t3268, t3310, t3318, t3324, t3327, t3332, t3334, t3352, t3357, t3360, t3369, t3371, t3376, t3378, t3396, t3401, t3404, t436);
    (t3383, t3390, t3395, t3396, t3399, t3400, t3401, t3402, t3403, t3404, t3407)
}
