//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1395/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1395<F: Float>(t28002: F, t8288: F, t898: F, t18509: F, t18513: F, t2297: F, t3806: F, t237: F, t27540: F, t27916: F, t27930: F, t27939: F, t27987: F, t27989: F, t27991: F, t27993: F, t27995: F, t27998: F, t28001: F) -> (F, F, F, F) {
    let t28005 = 0.10254018858216406658e4 * t898 * t28002 * t8288;
    let t28010 = 0.91082604192152556044e5 * t898 * t18509 * t3806 * t18513 * t2297;
    let t28012 = 0.19751673498613801407e-1 * t237 * t27540;
    let t28013 = -t27987 + t27989 + t27916 - t27991 + t27993 + t27995 + t27998 - t27930 - t28001 - t28005 - t27939 - t28010 + t28012;
    (t28005, t28010, t28012, t28013)
}
