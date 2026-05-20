//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1000/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1000<F: Float>(t11509: F, t23648: F, t981: F, t23448: F, t23450: F, t23461: F, t23463: F, t23465: F, t23469: F, t23549: F, t23552: F, t23554: F, t23556: F) -> (F, F) {
    let t23649 = t23648 * t11509;
    let t23651 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t23649;
    let t23652 = t23461 + t23463 + t23465 - t23469 + t23549 + t23552 - t23651 + t23448 - t23554 - t23556 - t23450;
    (t23651, t23652)
}
