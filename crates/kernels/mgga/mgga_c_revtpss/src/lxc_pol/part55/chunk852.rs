//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 852/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk852<F: Float>(t233: F, t28340: F, t1957: F, t2061: F, t231: F, t4423: F, t7076: F, t25317: F, t8006: F, t886: F, t4533: F, t7071: F, t27213: F, t7407: F, t1956: F, t26508: F, t26521: F, t26522: F, t26529: F, t26534: F, t26536: F, t26538: F, t27199: F, t4487: F, t7070: F, t7403: F, t7420: F) -> (F,) {
    let t28399 = t233 * t28340;
    let t28400 = t1957 * t28399;
    let t28404 = t2061 * t4423 * t231;
    let t28405 = t7076 * t28404;
    let t28411 = t25317 * t8006 * t886;
    let t28417 = t2061 * t4533;
    let t28418 = t7071 * t28417;
    let t28422 = t27213 * t7407;
    let t28424 = -0.4336814094102599731e0 * t1956 * t28400 + 0.4336814094102599731e0 * t7070 * t28405 + 0.4336814094102599731e0 * t27199 * t7420 - 0.26020884564615598386e1 * t7070 * t28411 + t26508 + 0.13170898365871023197e1 * t7403 * t4487 + t26521 - 0.12851425765524037203e-1 * t26522 + 0.8673628188205199462e0 * t7070 * t28418 + 0.72280234901709995518e-2 * t26529 - t26534 - t26536 - t26538 + 0.72280234901709995518e-2 * t28422;
    (t28424,)
}
