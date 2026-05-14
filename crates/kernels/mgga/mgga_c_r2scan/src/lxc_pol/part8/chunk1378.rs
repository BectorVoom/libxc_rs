//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1378/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1378<F: Float>(t5: F, t736: F, t9904: F, t22319: F, t22321: F, t22325: F, t22329: F, t26667: F, t26669: F, t26673: F, t26676: F, t26680: F, t26682: F, t26684: F, t26687: F, t26688: F) -> (F,) {
    let t33599 = t9904 * t5 * t736;
    let t33608 = t22319 + 0.1350520664e0 * t22321 - 0.35089341735807877242e1 * t22325 + 0.51947577317044391277e2 * t22329 - 0.54217906501508699211e-2 * t33599 + 0.18701127834135980859e4 * t26667 - 0.42107210082969452692e2 * t26669 - t26673 - t26676 - 0.10526802520742363173e2 * t26680 - 0.10526802520742363173e2 * t26682 + 0.31580407562227089518e2 * t26684 - 3.0 * t26687 - 72.0 * t26688;
    (t33608,)
}
