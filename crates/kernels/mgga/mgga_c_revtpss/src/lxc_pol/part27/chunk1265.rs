//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1265/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1265<F: Float>(t3924: F, t676: F, t25880: F, t25899: F, t10008: F, t1955: F, t2022: F, t9646: F, t9648: F, t25875: F, t94394: F, t94398: F) -> (F, F, F, F, F) {
    let t94639 = t676 * t3924;
    let t94640 = t25880 * t94639;
    let t94641 = t25899 * t94640;
    let t94643 = t1955 * t10008;
    let t94648 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94650 = t94649 * t94398;
    (t94640, t94641, t94643, t94648, t94650)
}
