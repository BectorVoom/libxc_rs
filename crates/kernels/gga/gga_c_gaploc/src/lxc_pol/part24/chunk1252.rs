//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1252/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1252<F: Float>(t32697: F, t10679: F, t10789: F, t1897: F, t29631: F, t32669: F, t32671: F, t32674: F, t32676: F, t32679: F, t32681: F, t32683: F, t32685: F, t32691: F, t32695: F, t5227: F, t5524: F, t5836: F) -> F {
    let t32698 = F::cast_from(0.96131577876777803547e-3_f64) * t32697;
    let t32701 = -F::cast_from(0.8545029144602471425e-3_f64) * t5524 * t10679 - t32669 - t32671 + t32674 + t32676 + t29631 - t32679 - t32681 - t32683 + t32685 + F::cast_from(0.46143157380853345702e-1_f64) * t1897 * t10789 * t5836 + t32691 - t32695 - t32698 + F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t10679;
    t32701
}
