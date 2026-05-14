//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1113/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1113<F: Float>(t33928: F, t33930: F, t33932: F, t33935: F, t33937: F, t33939: F, t33941: F, t33943: F, t33946: F, t33949: F, t33952: F, t33956: F, t33962: F, t33967: F, t33969: F, t33972: F, t33975: F, t33978: F, t33980: F, t33983: F, t33988: F, t33991: F) -> (F, F) {
    let t37938 = 0.45018799441230669486e-7 * t33928 - 0.2023819338830593704e-6 * t33930 + 0.22745373045674261828e-4 * t33932 + 0.5060221354166666667e-5 * t33935 + 0.45018799441230669486e-7 * t33937 + 0.45018799441230669488e-6 * t33939 + 0.66295654499063700024e-7 * t33941 + 0.13259130899812740005e-6 * t33943 + 0.66295654499063700024e-7 * t33946 - 0.27826035332451380868e-3 * t33949 - 0.13900948042322754167e-2 * t33952;
    let t37950 = -0.1348042442506961251e-6 * t33956 - 0.40083661544871514617e-6 * t33962 + 0.1672914890006736473e-7 * t33967 - 0.5060221354166666667e-5 * t33969 + 0.4637672555408563478e-4 * t33972 - 0.14339270485772026911e-8 * t33975 + 0.95595136571813512741e-9 * t33978 + 0.2318836277704281739e-4 * t33980 - 0.5462579232675057871e-9 * t33983 - 0.99511007074824895497e-6 * t33988 - 0.36620703859188537988e-5 * t33991;
    (t37938, t37950)
}
