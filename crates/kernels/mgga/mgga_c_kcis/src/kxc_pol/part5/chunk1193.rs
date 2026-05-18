//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1193/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1193<F: Float>(t19895: F, t19897: F, t19899: F, t19902: F, t19906: F, t19909: F, t19912: F, t19914: F, t19916: F, t19918: F, t19920: F, t19922: F, t19924: F, t19927: F, t19929: F, t19932: F, t19935: F, t19938: F, t19941: F) -> F {
    let t19943 = -t19895 / F::new(576.0) - t19897 / F::new(18.0) - t19899 / F::new(8.0) - t19902 / F::new(64.0) - t19906 / F::new(256.0) - t19909 / F::new(576.0) + t19912 / F::new(108.0) + t19914 / F::new(256.0) + t19916 / F::new(128.0) + t19918 / F::new(12.0) - t19920 / F::new(12.0) - t19922 / F::new(128.0) - t19924 / F::new(16.0) - t19927 / F::new(12.0) - t19929 / F::new(24.0) - F::new(3.0) / F::new(8.0) * t19932 + t19935 / F::new(72.0) + t19938 / F::new(54.0) - t19941 / F::new(96.0);
    t19943
}
