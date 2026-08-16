//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 876/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk876<F: Float>(t8778: F, t8782: F, t8790: F, t8794: F, t8796: F, t8799: F, t8802: F, t8805: F, t8811: F, t8815: F, t8818: F, t8823: F, t8825: F) -> F {
    let t10603 = -F::cast_from(0.11255061864162936194e-7_f64) * t8778 - F::cast_from(0.22510123728325872388e-7_f64) * t8782 + F::cast_from(0.16413631885237615283e-8_f64) * t8790 - F::cast_from(0.11255061864162936194e-6_f64) * t8794 + F::cast_from(0.88633612180283122527e-6_f64) * t8796 - F::cast_from(0.10317140042149358177e-4_f64) * t8799 + F::cast_from(0.24326659074064819792e-2_f64) * t8802 - F::cast_from(0.4048307291666666667e-4_f64) * t8805 - F::cast_from(0.82386285397499523032e-5_f64) * t8811 + F::cast_from(0.1349435763888888889e-4_f64) * t8815 + F::cast_from(0.27801896084645508334e-2_f64) * t8818 + F::cast_from(0.2748593934505475288e-6_f64) * t8823 - F::cast_from(0.67632724766374884052e-4_f64) * t8825;
    t10603
}
