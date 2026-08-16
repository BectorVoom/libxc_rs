//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2508/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2508<F: Float>(t14673: F, t40731: F, t40593: F, t4447: F, t4462: F, t10760: F, t40763: F, t4353: F, t1559: F, t775: F, t40834: F, t854: F) -> (F, F, F, F, F, F) {
    let t50604 = t40731 * t14673;
    let t50605 = F::cast_from(0.16262400898971305032e-2_f64) * t50604;
    let t50606 = t40593 * t4447;
    let t50607 = F::cast_from(0.17006693853500995666e-1_f64) * t50606;
    let t50608 = t40593 * t4462;
    let t50611 = t10760 * t40763 * t4353;
    let t50613 = t1559 * t775;
    let t50615 = t40834 * t854 * t50613;
    (t50605, t50607, t50608, t50611, t50613, t50615)
}
