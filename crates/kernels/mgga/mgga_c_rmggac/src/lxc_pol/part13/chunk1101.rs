//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1101/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1101<F: Float>(t1356: F, t29838: F, t36893: F, t38107: F, t42011: F, t42032: F, t42034: F, t42042: F, t42050: F, t42055: F, t42059: F, t43971: F, t43975: F, t43978: F, t43979: F, t43981: F, t43987: F, t43990: F, t4985: F, t8281: F) -> F {
    let t43993 = F::new(0.23942587439980034662e-4) * t42011 + F::new(0.11974241701863808564e0) * t4985 * t8281 + F::new(0.79828278012425390428e-1) * t1356 * t43971 - F::new(0.11974241701863808564e0) * t1356 * t43975 - t43978 - t43979 - F::new(0.39726959900411316772e-4) * t36893 + F::new(0.95793933614910468512e0) * t29838 * t43981 + F::new(0.30487649791575028312e-3) * t42032 - F::new(0.47896966807455234256e0) * t42034 + F::new(0.60975299583150056624e-3) * t42042 + t43987 - F::new(0.5107751987195740728e-4) * t42050 + F::new(0.5107751987195740728e-4) * t42055 + t43990 + F::new(0.5987120850931904282e-1) * t42059 - F::new(0.4726e1) * t38107;
    t43993
}
