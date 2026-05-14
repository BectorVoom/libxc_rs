//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1217/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1217<F: Float>(t1928: F, t5742: F, t990: F, t102308: F, t102311: F, t102313: F, t102334: F, t102337: F, t102340: F, t102348: F, t103063: F, t28369: F, t28375: F, t28388: F, t28392: F, t28420: F, t7911: F, t8155: F, t98294: F) -> (F,) {
    let t103445 = t5742 * t1928 * t990;
    let t103459 = -0.24712962962962962964e-2 * t28392 * t28420 - 0.185671721767578125e-4 * t28388 * t103063 + 0.12356481481481481481e-2 * t103445 * t7911 + 0.12356481481481481482e-2 * t98294 * t8155 - 0.58958024691358024689e-2 * t102308 + 0.11054629629629629629e-2 * t102311 - 0.27802083333333333334e-2 * t28369 * t28375 - 0.22109259259259259259e-2 * t102313 - 0.27636574074074074073e-2 * t102334 + 0.18424382716049382715e-2 * t102337 - 0.16581944444444444444e-1 * t102340 + 0.73697530864197530861e-2 * t102348;
    (t103459,)
}
