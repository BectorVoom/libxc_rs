//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 712/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk712<F: Float>(t127: F, t1533: F, t496: F, t5788: F, t5791: F, t5797: F, t5799: F, t5806: F, t5810: F, t5815: F, t5817: F, t5819: F, t5823: F, t5826: F, t5831: F, t5836: F, t5837: F) -> F {
    let t5841 = t5788 / F::cast_from(2.0_f64) - t496 * t5791 / F::cast_from(2.0_f64) + t5797 - F::cast_from(6.0_f64) * t496 * t5799 - F::cast_from(0.881424e1_f64) * t5806 - F::cast_from(0.293808e1_f64) * t5810 - t5815 - t5817 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5819 - t5823 - F::cast_from(0.293808e2_f64) * t127 * t5826 + t5831 + t5836 + F::cast_from(0.1762848e2_f64) * t127 * t5837 * t1533;
    t5841
}
