//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2351/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2351(t13532: f64, t13784: f64, t2986: f64, t10213: f64, t134: f64, t344: f64, t13537: f64, t4509: f64, t4540: f64, t13797: f64, t1597: f64, t10186: f64, t13848: f64) -> (f64, f64, f64, f64, f64) {
    let t48210 = t2986 * t13784 * t13532;
    let t48213 = t134 * t10213 * t344;
    let t48215 = t2986 * t48213 * t13537;
    let t48217 = t4509 * t4540;
    let t48221 = t13797 * t1597;
    let t48233 = t10186 * t13848;
    (t48210, t48215, t48217, t48221, t48233)
}
