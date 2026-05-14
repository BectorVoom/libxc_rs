//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 392/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk392<F: Float>(t1576: F, t1954: F, t178: F, t1936: F, t567: F, t647: F, t1939: F, t424: F, t668: F, t136: F, t5: F, t1033: F, t116: F, t188: F, t190: F) -> (F, F, F, F, F, F, F) {
    let t1955 = t1576 * t1954;
    let t1958 = t178 * t1936;
    let t1959 = t647 * t567;
    let t1960 = t1939 * t1959;
    let t1965 = t424 * t668;
    let t1968 = t136 * t5;
    let t1969 = t1968 * t1033;
    let t1970 = t116 * t1969;
    let t1971 = t188 * t190;
    (t1955, t1958, t1960, t1965, t1968, t1970, t1971)
}
