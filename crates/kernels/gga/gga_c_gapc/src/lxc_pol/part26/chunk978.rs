//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 978/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk978<F: Float>(t1089: F, t3687: F, t9906: F, t11945: F, t9895: F, t11878: F, t15805: F, t1936: F, t3775: F, t9980: F, t33831: F, t33834: F, t33836: F, t33838: F, t33840: F, t33842: F, t33847: F) -> (F,) {
    let t33850 = t9906 * t3687 * t1089;
    let t33852 = t9895 * t11945;
    let t33855 = t15805 * t1936 * t11878;
    let t33857 = t3775 * t9980;
    let t33859 = -0.34752370105806885418e-3 * t33831 + 0.12228868272569444445e-4 * t33834 - 0.2318836277704281739e-4 * t33836 - 0.90579542097823505428e-7 * t33838 + 0.60706991790943943129e-6 * t33840 - 0.10793703140429833089e-5 * t33842 + 0.92386400563397210585e-6 * t33847 - 0.16882049790461501058e-6 * t33850 + 0.27991498566271340012e-7 * t33852 + 0.10110318318802209383e-5 * t33855 - 0.57970906942607043474e-5 * t33857;
    (t33859,)
}
