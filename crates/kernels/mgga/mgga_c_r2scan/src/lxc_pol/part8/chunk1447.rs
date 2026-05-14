//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1447/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1447<F: Float>(t10409: F, t19131: F, t806: F, t2358: F, t9612: F, t10416: F, t1248: F, t10432: F, t19107: F, t810: F, t10413: F, t19093: F, t2369: F, t2373: F, t2911: F, t295: F, t305: F, t34919: F, t34923: F, t803: F, t9623: F, t9631: F, t997: F) -> (F, F, F, F, F) {
    let t34927 = t19131 * t10409 * t806;
    let t34930 = t2358 * t9612;
    let t34934 = t1248 * t10416 * t806;
    let t34946 = t19107 * t10432 * t810;
    let t34951 = -t19093 + 10.0 / 3.0 * t305 * t34919 + 10.0 / 9.0 * t305 * t34923 + 40.0 / 81.0 * t295 * t34927 + 10.0 / 3.0 * t295 * t34930 + 10.0 / 9.0 * t295 * t34934 + 400.0 / 27.0 * t2911 * t2369 - 200.0 / 9.0 * t2911 * t2373 + 50.0 / 27.0 * t997 * t9623 - 50.0 / 9.0 * t997 * t9631 + 40.0 / 81.0 * t305 * t34946 - 50.0 / 9.0 * t803 * t10413;
    (t34927, t34930, t34934, t34946, t34951)
}
