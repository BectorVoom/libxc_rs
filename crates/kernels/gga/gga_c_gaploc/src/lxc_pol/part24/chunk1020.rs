//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1020/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1020<F: Float>(t11005: F, t1445: F, t8556: F, t955: F, t10010: F, t10015: F, t3447: F, t4673: F, t2103: F, t4752: F, t948: F, t3025: F) -> (F, F, F, F, F, F, F, F) {
    let t11006 = t1445 * t11005;
    let t11010 = F::new(0.23833659967900284446e0) * t955 * t8556;
    let t11011 = F::new(0.31952438294933958064e-1) * t10010;
    let t11012 = F::new(0.31952438294933958064e-1) * t10015;
    let t11013 = t4673 * t3447;
    let t11015 = F::new(0.47667319935800568892e0) * t2103 * t11013;
    let t11016 = t4752 * t948;
    let t11018 = F::new(0.7150097990370085334e0) * t3025 * t11016;
    (t11006, t11010, t11011, t11012, t11013, t11015, t11016, t11018)
}
