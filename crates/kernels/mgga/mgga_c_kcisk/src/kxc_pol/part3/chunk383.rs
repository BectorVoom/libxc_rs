//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 383/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk383<F: Float>(t1944: F, t740: F, t748: F, t1872: F, t1894: F, t747: F, t746: F, t1757: F, t641: F, t741: F, t1932: F, t1938: F, t1942: F) -> (F, F, F, F, F, F, F, F) {
    let t1945 = t1944 * t740;
    let t1946 = t1945 * t748;
    let t1948 = t1872 * t740;
    let t1949 = t747 * t1894;
    let t1950 = t746 * t1949;
    let t1951 = t1948 * t1950;
    let t1953 = t641 * t1757;
    let t1954 = t746 * t1953;
    let t1955 = t741 * t1954;
    let t1957 = t1932 / F::new(16.0) - t1938 / F::new(16.0) + t1942 / F::new(24.0) - t1946 / F::new(256.0) + t1951 / F::new(256.0) - t1955 / F::new(192.0);
    (t1945, t1946, t1948, t1950, t1951, t1954, t1955, t1957)
}
