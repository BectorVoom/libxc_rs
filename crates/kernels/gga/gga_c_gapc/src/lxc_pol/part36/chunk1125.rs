//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1125/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1125<F: Float>(t34995: F, t35001: F, t35003: F, t35005: F, t35007: F, t35010: F, t35013: F, t35016: F, t35019: F, t35024: F, t35027: F, t35031: F, t35034: F, t35036: F, t35039: F, t35041: F, t35045: F, t35048: F, t35051: F, t35056: F, t35059: F, t35062: F) -> (F, F) {
    let t38371 = 0.43174812561719332356e-5 * t34995 - 0.48751922435761895589e-4 * t35001 + 0.18550690221634253912e-3 * t35003 - 0.18550690221634253912e-3 * t35005 + 0.19808908880926767702e-4 * t35007 - 0.90040494913303489554e-6 * t35010 - 0.15458908518028544927e-5 * t35013 - 0.26519114751114692796e-6 * t35016 + 0.16009199995585360443e-7 * t35019 - 0.99511007074824895497e-6 * t35024 + 0.12310223913928211462e-7 * t35027;
    let t38384 = -0.80966145833333333339e-4 * t35031 - 0.8839704917038230932e-7 * t35034 - 0.10136107947527008247e-2 * t35036 - 0.50603841145833333338e-5 * t35039 - 0.11594181388521408695e-4 * t35041 - 0.77347418024084520656e-8 * t35045 - 0.16193229166666666668e-3 * t35048 - 0.1969635826228513834e-6 * t35051 + 0.31433990684987949196e-7 * t35056 + 0.84412963981222021456e-7 * t35059 - 0.19808908880926767702e-4 * t35062;
    (t38371, t38384)
}
