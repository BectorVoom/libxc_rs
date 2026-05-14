//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 305/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk305<F: Float>(t118: F, t1944: F, t1756: F, t338: F, t1927: F, t1929: F, t1931: F, t1934: F, t1937: F, t1939: F, t1942: F, t82: F, t1281: F, t1283: F, t1287: F) -> (F, F, F, F, F, F) {
    let t1945 = t118 * t1944;
    let t1947 = t338 * t1756;
    let t1948 = t118 * t1947;
    let t1950 = -0.11974241701863808564e0 * t1927 + 0.35922725105591425692e0 * t1929 + 0.11974241701863808564e0 * t1931 - 0.59871208509319042821e-1 * t1934 - 0.23948483403727617128e0 * t1937 - 0.11974241701863808564e0 * t1939 + 0.59871208509319042821e-1 * t1942 - 0.39914139006212695214e-1 * t1945 + 0.19957069503106347607e-1 * t1948;
    let t1951 = t82 * t1950;
    let t1953 = -t1281 - t1283 - t1287;
    (t1945, t1947, t1948, t1950, t1951, t1953)
}
