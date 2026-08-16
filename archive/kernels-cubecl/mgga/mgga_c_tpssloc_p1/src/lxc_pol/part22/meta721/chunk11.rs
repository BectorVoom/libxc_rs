//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2355/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2355<F: Float>(t10054: F, t1499: F, t1525: F, t16754: F, t16805: F, t17023: F, t20853: F, t20854: F, t20857: F, t20858: F, t20861: F, t20862: F, t20937: F, t2617: F, t2732: F, t40917: F, t4166: F, t4298: F, t5575: F, t812: F, t863: F) -> F {
    let t68299 = F::cast_from(6.0_f64) * t10054 * t20861 * t812 - t20853 * t2732 * t812 - F::cast_from(6.0_f64) * t20857 * t40917 * t812 + F::cast_from(3.0_f64) * t1499 * t17023 + F::cast_from(3.0_f64) * t1525 * t16805 - F::cast_from(3.0_f64) * t16754 * t4166 - t20854 * t2617 - F::cast_from(6.0_f64) * t20858 * t2617 + F::cast_from(6.0_f64) * t20862 * t2617 + t20937 * t863 + F::cast_from(3.0_f64) * t4298 * t5575;
    t68299
}
