//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 987/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk987<F: Float>(t7990: F, t8081: F, t2131: F, t2147: F, t463: F, t8099: F, t323: F, t3242: F, t633: F, t32092: F, t8313: F, t30029: F, t8310: F) -> (F, F, F, F, F) {
    let t33210 = t7990 * t8081;
    let t33214 = t2131 * t2147 * t8099 * t463;
    let t33227 = F::cast_from(0.19756347548806534796e1_f64) * t3242 * t633 * t323;
    let t33228 = t32092 * t8313;
    let t33230 = t30029 * t8310;
    (t33210, t33214, t33227, t33228, t33230)
}
