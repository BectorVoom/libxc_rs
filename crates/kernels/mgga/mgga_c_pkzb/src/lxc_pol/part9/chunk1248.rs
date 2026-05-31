//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1248/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1248<F: Float>(t18039: F, t18063: F, t18067: F, t18071: F, t18073: F, t18079: F, t18084: F, t18089: F, t21807: F, t21822: F, t2899: F, t2919: F, t2922: F, t301: F, t5704: F, t5945: F, t5961: F, t757: F, t758: F, t761: F, t7707: F, t7770: F) -> F {
    let t21837 = F::cast_from(0.14291339372689912324e-3_f64) * t18039 + F::cast_from(0.27439371595564631662e-1_f64) * t7707 * t7770 + F::cast_from(0.12862205435420921092e-2_f64) * t2922 * t21807 * t5961 - F::cast_from(0.25724410870841842183e-2_f64) * t2899 * t21807 * t5704 + F::cast_from(0.21437009059034868486e-3_f64) * t757 * t758 * t301 * t21822 * t761 + F::cast_from(0.21722835846488666732e-1_f64) * t5945 * t2919 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t18063 + t18067 / F::cast_from(144.0_f64) - t18071 / F::cast_from(288.0_f64) - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t18073 - t18079 / F::cast_from(18.0_f64) - t18084 / F::cast_from(27.0_f64) - t18089 / F::cast_from(72.0_f64);
    t21837
}
