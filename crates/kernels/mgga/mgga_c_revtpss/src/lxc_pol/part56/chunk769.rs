//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 769/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk769<F: Float>(t212: F, t7910: F, t1358: F, t689: F, t7925: F, t25904: F, t25899: F, t2022: F, t5774: F, t7296: F, t1955: F, t5710: F, t27960: F, t545: F, t2028: F, t1904: F, t2027: F, t2030: F, t26062: F, t26065: F, t26067: F, t26071: F, t26073: F, t26084: F, t5728: F, t7279: F, t7292: F, t7295: F, t7308: F, t7917: F, t7930: F) -> (F, F) {
    let t27985 = t212 * t7910;
    let t27986 = t27985 * t1358;
    let t27987 = t689 * t27986;
    let t27989 = t7925 * t689;
    let t27990 = t25904 * t27989;
    let t27992 = t25899 * t27989;
    let t28002 = t2022 * t5774;
    let t28003 = t7296 * t28002;
    let t28008 = t1955 * t5710;
    let t28011 = t545 * t27960;
    let t28012 = t2028 * t28011;
    let t28017 = -0.54878743191129263322e-2 * t27987 - 0.72280234901709995518e-2 * t27990 + 0.12851425765524037203e-1 * t27992 + 0.13170898365871023197e1 * t7279 * t5728 - 0.65854491829355115987e0 * t26084 * t1904 + 0.54878743191129263322e-2 * t26062 + 0.9757440539382783019e-2 * t26065 - 0.12851425765524037203e-1 * t26067 - t26071 + 0.72280234901709995518e-2 * t26073 + 0.8673628188205199462e0 * t7295 * t28003 - 0.4336814094102599731e0 * t7917 * t7308 - 0.4336814094102599731e0 * t28008 * t2030 - 0.4336814094102599731e0 * t2027 * t28012 - 0.4336814094102599731e0 * t7292 * t7930;
    (t28012, t28017)
}
