//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 935/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk935<F: Float>(t2211: F, t27059: F, t35625: F, t35629: F, t35633: F, t37786: F, t37787: F, t37788: F, t37789: F, t37790: F, t40172: F, t40177: F, t40182: F, t40185: F, t40188: F, t40191: F, t40194: F, t40198: F, t40201: F, t739: F) -> (F,) {
    let t43318 = 0.11974241701863808564e0 * t739 * t2211 * t27059 - 0.5107751987195740728e-4 * t40172 + 0.5107751987195740728e-4 * t40177 + 0.1702583995731913576e-4 * t40182 - 0.2727466165424534173e0 * t40185 + 0.8182398496273602519e0 * t40188 + 0.16364796992547205038e0 * t40191 + 0.40911992481368012596e-1 * t40194 - 0.86737941314158990616e-4 * t40198 + 0.162600798888400151e-2 * t40201 + t37786 - t37787 + t37788 - t37789 + t37790 + 0.1440846329149835838e-2 * t35625 + 0.12195059916630011325e-2 * t35629 + 0.12195059916630011325e-2 * t35633;
    (t43318,)
}
