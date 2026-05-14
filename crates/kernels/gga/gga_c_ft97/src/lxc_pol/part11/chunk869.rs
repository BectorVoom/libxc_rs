//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 869/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk869<F: Float>(t8157: F, t8935: F, t37931: F, t8873: F, t1701: F, t2059: F, t7883: F, t12374: F, t1683: F, t1992: F, t2001: F, t2030: F, t2032: F, t2043: F, t2057: F, t2060: F, t3347: F, t3392: F, t39835: F, t555: F, t5818: F, t8825: F, t8833: F, t8866: F, t8998: F) -> (F, F) {
    let t40106 = t8935 * t8157;
    let t40111 = t8873 * t37931;
    let t40123 = t1701 * t7883 * t2059;
    let t40128 = 24.0 * t1992 * t2060 + 24.0 * t2001 * t2057 * t2030 * t2059 - 0.65177969127962413846e0 * t40106 * t555 - 24.0 * t12374 * t8866 + 0.12383814134312858631e2 * t5818 * t40111 - 0.4127938044770952877e1 * t3392 * t40111 + 8.0 * t3347 * t8998 + 0.2416365355361531912e1 * t2043 * t39835 + 0.45910941751869106328e2 * t8825 * t1683 - 0.45910941751869106328e2 * t8833 * t40123 - 0.45910941751869106328e2 * t2032 * t1683;
    (t40123, t40128)
}
