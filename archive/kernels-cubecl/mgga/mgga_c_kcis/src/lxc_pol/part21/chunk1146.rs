//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1146/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1146<F: Float>(t27856: F, t7690: F, t2175: F, t26685: F, t26739: F, t26837: F, t26838: F, t26846: F, t26856: F, t26860: F, t27911: F, t27915: F, t27941: F, t27947: F, t27950: F, t27954: F, t27958: F, t27964: F, t27967: F, t27969: F, t27972: F, t27975: F, t7703: F, t8034: F) -> F {
    let t27981 = t7690 * t27856;
    let t27983 = -F::cast_from(0.66327777777777777776e-2_f64) * t27941 - F::cast_from(0.69505208333333333333e-3_f64) * t7703 * t27911 - t26837 - F::cast_from(0.44218518518518518517e-2_f64) * t26838 - F::cast_from(0.16581944444444444444e-2_f64) * t26846 + F::cast_from(0.16581944444444444444e-2_f64) * t27947 - F::cast_from(0.30891203703703703704e-3_f64) * t7703 * t27950 + F::cast_from(0.23168402777777777778e-3_f64) * t7703 * t27954 + F::cast_from(0.23168402777777777778e-3_f64) * t7703 * t27958 + F::cast_from(0.30918233506944444444e-4_f64) * t26685 * t27958 + t26856 - F::cast_from(0.23168402777777777778e-3_f64) * t26860 + F::cast_from(0.18534722222222222222e-2_f64) * t27964 * t2175 - F::cast_from(0.23168402777777777778e-3_f64) * t27967 - F::cast_from(0.16581944444444444444e-2_f64) * t27969 + F::cast_from(0.66327777777777777776e-2_f64) * t27972 + F::cast_from(0.23168402777777777778e-3_f64) * t27975 + F::cast_from(0.92754700520833333333e-4_f64) * t7690 * t27915 - F::cast_from(0.24734586805555555555e-3_f64) * t26739 * t8034 + F::cast_from(0.30918233506944444444e-4_f64) * t27981;
    t27983
}
