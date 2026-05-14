//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 978/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk978<F: Float>(t1250: F, t9565: F, t1014: F, t7723: F, t2179: F, t3169: F, t303: F, t2865: F, t355: F, t359: F, t342: F, t2180: F, t3245: F, t7732: F, t3183: F, t356: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26823 = t9565 * t1250;
    let t26826 = t1014 * t7723;
    let t26828 = t3169 * t2179;
    let t26829 = t303 * t26828;
    let t26832 = t355 * t2865 * t359;
    let t26833 = t342 * t26832;
    let t26834 = t303 * t26833;
    let t26836 = t3245 * t2180;
    let t26837 = 0.55273148148148148147e-3 * t26836;
    let t26838 = t1014 * t7732;
    let t26840 = t356 * t3183;
    (t26823, t26826, t26828, t26829, t26832, t26833, t26834, t26836, t26837, t26838, t26840)
}
