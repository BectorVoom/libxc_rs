//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1195/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1195<F: Float>(t15808: F, t2066: F, t28589: F, t5919: F, t22361: F, t27520: F, t12266: F, t7318: F, t102937: F, t102939: F, t102942: F, t102944: F, t102946: F, t102948: F, t102950: F, t3738: F, t7287: F) -> (F, F, F, F, F, F) {
    let t102952 = t15808 * t2066;
    let t102954 = t28589 * t5919;
    let t102956 = t27520 * t22361;
    let t102958 = t12266 * t7318;
    let t102960 = -0.125e0 * t102937 + 0.91666666666666666667e0 * t102939 + 0.61111111111111111111e0 * t102942 + 0.4046875e-1 * t102944 - 0.5e0 * t102946 - 0.21583333333333333334e0 * t102948 - 0.625e-1 * t102950 - 0.53958333333333333334e-1 * t102952 - 0.125e0 * t102954 + 0.1875e0 * t102956 - 0.4046875e-1 * t102958;
    let t102963 = t3738 * t7287;
    (t102952, t102954, t102956, t102958, t102960, t102963)
}
