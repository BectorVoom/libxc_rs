//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1871/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1871<F: Float>(t1096: F, t4975: F, t27651: F, t27638: F, t3143: F, t1983: F, t27642: F, t4983: F, t1984: F, t27543: F, t359: F, t1646: F, t7135: F) -> (F, F, F, F, F, F, F) {
    let t27664 = t4975 * t1096;
    let t27665 = t27651 * t27664;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27670 = t27642 * t4983;
    let t27676 = t1984 * t359 * t27543;
    let t27679 = t7135 * t1646;
    (t27664, t27665, t27668, t27669, t27670, t27676, t27679)
}
