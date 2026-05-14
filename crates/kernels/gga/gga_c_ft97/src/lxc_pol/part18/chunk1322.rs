//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1322/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1322<F: Float>(t12986: F, t5916: F, t23667: F, t27142: F, t23657: F, t23671: F, t27123: F, t379: F, t26909: F, t5899: F, t105505: F, t105508: F, t105511: F, t105514: F, t105517: F, t105520: F, t105524: F, t96066: F) -> (F, F, F, F, F, F) {
    let t105526 = t5916 * t12986;
    let t105528 = t27142 * t23667 * t105526;
    let t105532 = t23657 * t23671 * t27123 * t379;
    let t105534 = t26909 * t379;
    let t105536 = t5899 * t23667 * t105534;
    let t105538 = -t105505 - 6.0 * t105508 - t105511 - t105514 / 3.0 + t105517 + 2.0 / 3.0 * t105520 - t105524 / 6.0 + t96066 - 4.0 / 3.0 * t105528 - t105532 / 6.0 - 2.0 / 3.0 * t105536;
    (t105526, t105528, t105532, t105534, t105536, t105538)
}
