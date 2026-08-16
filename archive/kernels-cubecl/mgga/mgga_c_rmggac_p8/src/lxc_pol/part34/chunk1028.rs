//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1028/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1028<F: Float>(t77840: F, t14516: F, t8526: F, t2329: F, t71882: F, t76148: F, t71852: F, t71854: F, t76141: F, t77830: F, t77833: F, t77835: F, t77836: F, t77837: F, t77839: F) -> F {
    let t77841 = F::cast_from(0.10227998120342003148e-1_f64) * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = F::cast_from(0.10227998120342003148e-1_f64) * t77842;
    let t77844 = t71882 * t2329;
    let t77845 = F::cast_from(0.13637330827122670864e-1_f64) * t77844;
    let t77846 = F::cast_from(0.40911992481368012596e-1_f64) * t76148;
    let t77847 = -t71852 - t76141 + t71854 - t77830 - t77833 - t77835 - t77836 - t77837 - t77839 + t77841 + t77843 - t77845 + t77846;
    t77847
}
