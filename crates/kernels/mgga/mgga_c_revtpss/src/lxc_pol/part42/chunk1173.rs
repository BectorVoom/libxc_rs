//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1173/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1173<F: Float>(t359: F, t6343: F, t999: F, t1086: F, t6235: F, t1647: F, t4995: F, t3153: F, t6299: F, t4983: F, t4998: F, t19482: F, t19501: F, t1089: F, t1678: F, t4866: F) -> (F, F, F, F, F, F, F, F) {
    let t19556 = t359 * t6343;
    let t19557 = t19556 * t999;
    let t19566 = t6235 * t1086;
    let t19569 = t1647 * t4995;
    let t19572 = t6299 * t3153;
    let t19573 = t19572 * t4983;
    let t19576 = t19572 * t4998;
    let t19579 = t19482 * t999;
    let t19580 = t19501 * t19579;
    let t19584 = t1678 * t4866 * t1089;
    (t19557, t19566, t19569, t19572, t19573, t19576, t19580, t19584)
}
