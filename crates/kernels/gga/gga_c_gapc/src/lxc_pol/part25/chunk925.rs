//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 925/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk925<F: Float>(t8830: F, t8833: F, t8835: F, t8844: F, t8849: F, t8854: F, t8856: F, t8859: F, t8861: F, t8865: F, t8867: F, t8870: F, t8873: F) -> F {
    let t10619 = -F::new(0.12974218172834570556e-1) * t8830 - F::new(0.20241536458333333336e-3) * t8833 + F::new(0.2162369695472428426e-1) * t8835 + F::new(0.20011499994481700554e-7) * t8844 + F::new(0.16882592796244404291e-6) * t8849 + F::new(0.40022999988963401107e-7) * t8854 + F::new(0.39476761752968521453e-4) * t8856 - F::new(0.4637672555408563478e-4) * t8859 - F::new(0.21642471925239962898e-3) * t8861 - F::new(0.16882592796244404291e-6) * t8865 - F::new(0.33765185592488808582e-6) * t8867 - F::new(0.20011499994481700554e-7) * t8870 - F::new(0.98481791311425691698e-7) * t8873;
    t10619
}
