//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1185/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1185<F: Float>(t29608: F, t7974: F, t2012: F, t303: F, t5871: F, t1014: F, t29307: F, t102151: F, t102170: F, t102221: F, t102286: F, t102575: F, t12617: F, t27567: F, t27583: F, t28765: F, t94669: F, t94966: F, t98942: F, t98946: F, t99678: F) -> (F, F, F) {
    let t102764 = t29608 * t7974;
    let t102767 = t303 * t5871 * t2012;
    let t102769 = t1014 * t29307;
    let t102775 = -0.20612155671296296296e-4 * t99678 - 0.30918233506944444445e-4 * t27567 * t102286 - 0.38691203703703703703e-3 * t94669 + 0.185671721767578125e-4 * t94966 * t102170 - 0.23168402777777777778e-3 * t27583 * t102221 + 0.15445601851851851852e-3 * t27583 * t12617 * t28765 * t102575 + 0.61782407407407407407e-3 * t102764 + 0.23214722222222222222e-2 * t102767 + 0.77382407407407407407e-3 * t102769 + 0.20635308641975308642e-2 * t98942 + 0.15459116753472222222e-4 * t27567 * t102151 - 0.41270617283950617283e-2 * t98946;
    (t102767, t102769, t102775)
}
