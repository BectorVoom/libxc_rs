//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1244/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1244<F: Float>(t22572: F, t23705: F, t30079: F, t105207: F, t105211: F, t105224: F, t118714: F, t118852: F, t119056: F, t1737: F, t1742: F, t2036: F, t23715: F, t26674: F, t26678: F, t30038: F, t4711: F, t5570: F, t5785: F, t5790: F, t5791: F, t61607: F, t61654: F) -> (F,) {
    let t119092 = t23705 * t22572 * t30079;
    let t119107 = -0.54738951849294959987e0 * t61607 * t5791 + 0.4445200072839506173e-1 * t105207 + t105211 - t105224 - 0.4445200072839506173e-1 * t23715 * t5570 * t1737 * t119056 + 0.22226000364197530865e-1 * t119092 - 0.90613700826057446696e0 * t61654 * t26678 - 0.45306850413028723348e0 * t26674 * t30038 - 0.66678001092592592596e-1 * t23705 * t5570 * t1742 * t118714 - 0.24163653553615319119e1 * t5785 * t118852 + 0.27369475924647479994e0 * t2036 * t5790 * t4711;
    (t119107,)
}
