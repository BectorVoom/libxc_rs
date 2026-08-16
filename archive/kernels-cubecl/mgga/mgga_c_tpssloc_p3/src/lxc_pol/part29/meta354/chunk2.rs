//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1432/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1432<F: Float>(t103: F, t584: F, t16: F, t4063: F, t100: F, t12771: F, t12774: F, t12775: F, t12778: F, t12781: F, t12784: F, t12792: F, t12795: F, t12796: F, t12799: F, t1445: F, t1447: F, t2336: F, t2351: F, t2355: F, t4050: F, t4054: F, t657: F, t92: F) -> F {
    let t12802 = t103 * t584;
    let t12805 = t4063 * t16;
    let t12808 = F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t2336 * t1445 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t657 * t4050 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t657 * t4054 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t92 * t12771 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t12774 * t12775 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t92 * t12778 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t12781 - F::cast_from(5.0_f64) * t92 * t12784 - F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t1447 * t2351 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t1447 * t2355 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t100 * t12792 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t12795 * t12796 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t100 * t12799 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t12802 + F::cast_from(5.0_f64) * t100 * t12805;
    t12808
}
