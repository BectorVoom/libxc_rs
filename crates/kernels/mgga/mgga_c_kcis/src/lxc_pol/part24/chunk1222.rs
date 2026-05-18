//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1222/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1222<F: Float>(t99904: F, t99906: F, t99908: F, t99910: F, t99912: F, t99914: F, t99917: F, t99919: F, t99921: F, t99923: F, t99925: F, t99927: F, t99929: F, t99931: F, t99933: F, t99935: F, t99937: F, t99939: F, t99941: F) -> F {
    let t99943 = t99904 / F::new(432.0) + t99906 / F::new(64.0) - t99908 / F::new(8.0) - t99910 / F::new(24.0) + t99912 / F::new(12.0) + t99914 / F::new(4.0) + t99917 / F::new(24.0) - t99919 / F::new(288.0) + t99921 / F::new(48.0) + t99923 / F::new(96.0) + t99925 / F::new(128.0) - t99927 / F::new(96.0) + t99929 / F::new(8.0) + t99931 / F::new(18.0) - t99933 / F::new(72.0) + F::new(2.0) / F::new(9.0) * t99935 + t99937 / F::new(64.0) - t99939 / F::new(64.0) - t99941 / F::new(24.0);
    t99943
}
