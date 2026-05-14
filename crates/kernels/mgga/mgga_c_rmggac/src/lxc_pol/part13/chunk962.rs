//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 962/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk962<F: Float>(t1347: F, t2475: F, t41828: F, t41882: F, t41884: F, t1550: F, t36680: F, t36715: F, t38047: F, t41834: F, t41836: F, t41838: F, t41846: F, t41848: F, t41850: F, t41863: F, t41865: F, t41887: F, t5207: F, t699: F) -> (F,) {
    let t43877 = t1347 * t2475;
    let t43878 = 0.39726959900411316772e-4 * t41828;
    let t43891 = 0.39726959900411316772e-4 * t41882;
    let t43892 = 0.39726959900411316772e-4 * t41884;
    let t43895 = t43877 - t43878 + 0.5107751987195740728e-4 * t41834 - 0.16364796992547205038e0 * t41836 - 0.40911992481368012596e-1 * t41838 - 0.11974241701863808564e0 * t1550 * t699 * t5207 - 0.47896966807455234256e0 * t36680 - 0.1064114997332445985e-4 * t41846 - 0.5987120850931904282e-1 * t41848 - 0.11974241701863808564e0 * t41850 - 0.85129199786595678799e-5 * t41863 - 0.1702583995731913576e-4 * t41865 - t38047 + t43891 + t43892 + 0.2727466165424534173e-1 * t41887 - 0.10909864661698136692e0 * t36715;
    (t43895,)
}
