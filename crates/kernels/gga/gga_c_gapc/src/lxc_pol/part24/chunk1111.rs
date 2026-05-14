//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1111/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1111<F: Float>(t34036: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F, t34056: F, t34060: F, t34062: F, t34066: F, t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34092: F, t34095: F, t34098: F, t34100: F) -> (F, F) {
    let t37976 = -0.23333242910879629631e-3 * t34036 + 0.2318836277704281739e-4 * t34038 - 0.12309827972211511188e-7 * t34043 - 0.6154913986105755594e-8 * t34046 + 0.39777392699438220015e-6 * t34048 - 0.41267232821128113697e-4 * t34050 + 0.6403474032520650428e-6 * t34052 + 0.57211390956563975807e-5 * t34054 + 0.2813674965076916843e-7 * t34056 + 0.93146396372185817726e-9 * t34060 + 0.99041358770707472872e-5 * t34062;
    let t37989 = -0.13505639832369200846e-5 * t34066 - 0.8004342540650813035e-7 * t34069 - 0.80189736504692130024e-6 * t34071 - 0.5238829942192678162e-8 * t34075 - 0.64454108540756375024e-8 * t34079 + 0.12144531250000000001e-2 * t34084 + 0.17678841199750320007e-7 * t34088 - 0.19676021349741883234e-7 * t34092 - 0.13505639832369200846e-5 * t34095 + 0.15716489826578034486e-7 * t34098 - 0.7246363367825880434e-6 * t34100;
    (t37976, t37989)
}
