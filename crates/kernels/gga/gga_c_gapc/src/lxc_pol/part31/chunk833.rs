//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 833/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk833<F: Float>(t8778: F, t8782: F, t8790: F, t8794: F, t8796: F, t8799: F, t8802: F, t8805: F, t8811: F, t8815: F, t8818: F, t8823: F, t8825: F, t8830: F, t8833: F, t8835: F, t8844: F, t8849: F, t8854: F, t8856: F, t8859: F, t8861: F, t8865: F, t8867: F, t8870: F, t8873: F) -> (F, F) {
    let t10603 = -0.11255061864162936194e-7 * t8778 - 0.22510123728325872388e-7 * t8782 + 0.16413631885237615283e-8 * t8790 - 0.11255061864162936194e-6 * t8794 + 0.88633612180283122527e-6 * t8796 - 0.10317140042149358177e-4 * t8799 + 0.24326659074064819792e-2 * t8802 - 0.4048307291666666667e-4 * t8805 - 0.82386285397499523032e-5 * t8811 + 0.1349435763888888889e-4 * t8815 + 0.27801896084645508334e-2 * t8818 + 0.2748593934505475288e-6 * t8823 - 0.67632724766374884052e-4 * t8825;
    let t10619 = -0.12974218172834570556e-1 * t8830 - 0.20241536458333333336e-3 * t8833 + 0.2162369695472428426e-1 * t8835 + 0.20011499994481700554e-7 * t8844 + 0.16882592796244404291e-6 * t8849 + 0.40022999988963401107e-7 * t8854 + 0.39476761752968521453e-4 * t8856 - 0.4637672555408563478e-4 * t8859 - 0.21642471925239962898e-3 * t8861 - 0.16882592796244404291e-6 * t8865 - 0.33765185592488808582e-6 * t8867 - 0.20011499994481700554e-7 * t8870 - 0.98481791311425691698e-7 * t8873;
    (t10603, t10619)
}
