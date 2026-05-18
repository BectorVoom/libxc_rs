//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 879/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk879<F: Float>(t8778: F, t8782: F, t8790: F, t8794: F, t8796: F, t8799: F, t8802: F, t8805: F, t8811: F, t8815: F, t8818: F, t8823: F, t8825: F) -> F {
    let t10603 = -F::new(0.11255061864162936194e-7) * t8778 - F::new(0.22510123728325872388e-7) * t8782 + F::new(0.16413631885237615283e-8) * t8790 - F::new(0.11255061864162936194e-6) * t8794 + F::new(0.88633612180283122527e-6) * t8796 - F::new(0.10317140042149358177e-4) * t8799 + F::new(0.24326659074064819792e-2) * t8802 - F::new(0.4048307291666666667e-4) * t8805 - F::new(0.82386285397499523032e-5) * t8811 + F::new(0.1349435763888888889e-4) * t8815 + F::new(0.27801896084645508334e-2) * t8818 + F::new(0.2748593934505475288e-6) * t8823 - F::new(0.67632724766374884052e-4) * t8825;
    t10603
}
