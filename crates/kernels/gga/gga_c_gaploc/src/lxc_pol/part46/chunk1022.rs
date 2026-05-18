//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1022/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1022<F: Float>(t10800: F, t2728: F, t1960: F, t3511: F, t13166: F, t1955: F, t42501: F, t42503: F, t42506: F, t42509: F, t42512: F, t42514: F, t42516: F, t42518: F, t43410: F, t43460: F, t43524: F, t43583: F, t43637: F, t43687: F, t43747: F, t43794: F, t43859: F, t43899: F, t43948: F, t44006: F, t44061: F, t44108: F, t44153: F, t44188: F, t44194: F, t44196: F, t44198: F, t44202: F, t44203: F, t44207: F, t748: F) -> F {
    let t44208 = t10800 * t2728;
    let t44211 = t1960 * t3511 * t2728;
    let t44213 = -t748 * (t43410 + t43460 + t43524 + t43583 + t43637 + t43687 + t43747 + t43794 + t43859 + t43899 + t43948 + t44006 + t44061 + t44108 + t44153 + t44188) - t42501 - t42503 - t44194 - t42506 - t1955 * t13166 + F::new(4.0) * t44196 - F::new(2.0) * t44198 + t44202 - F::new(2.0) * t44203 - t42509 - t42512 - t44207 - F::new(2.0) * t44208 + t42514 + t42516 + F::new(4.0) * t44211 - t42518;
    t44213
}
