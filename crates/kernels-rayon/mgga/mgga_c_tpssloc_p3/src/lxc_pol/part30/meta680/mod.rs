//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta680 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2134;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2135;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2136;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta680(t28117: f64, t81159: f64, t1377: f64, t6330: f64, t1385: f64, t22635: f64, t26331: f64, t26332: f64, t5187: f64, t19885: f64, t90915: f64, t91004: f64, t28135: f64, t6914: f64, t1992: f64, t550: f64, t57607: f64, t6976: f64, t28168: f64, t57704: f64, t562: f64, t6347: f64, t1307: f64, t26446: f64, t57545: f64, t90750: f64, t90760: f64, t90782: f64, t90789: f64, t90792: f64, t90795: f64, t90798: f64, t90806: f64, t90807: f64, t93517: f64, t19893: f64, t90914: f64, t1799: f64, t1834: f64, t1352: f64, t22633: f64, t19743: f64, t3807: f64, t20014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96920, t96925, t96929, t96935) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2134(t28117, t81159, t1377, t6330, t1385, t22635, t26331, t26332, t5187, t19885, t90915, t91004);
        let (t96937, t96941, t96945, t96949, t96951, t96954) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2135(t28135, t6914, t1992, t550, t57607, t6976, t28168, t57704, t562, t6347, t1307, t26331, t26446);
        let t96960 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2136(t1992, t550, t57545, t6976, t90750, t90760, t90782, t90789, t90792, t90795, t90798, t90806, t90807, t93517, t96935, t96937, t96941, t96945, t96949, t96954);
        let (t96962, t96964, t96967, t96972, t96976, t96979) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2137(t19893, t90914, t90915, t1799, t1834, t1352, t22633, t6976, t96951, t19743, t3807, t1992, t20014);
    (t96920, t96925, t96929, t96960, t96962, t96964, t96967, t96972, t96976, t96979)
}
