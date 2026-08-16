//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1606/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1606<F: Float>(t1246: F, t14985: F, t1235: F, t5011: F, t5072: F, t5079: F, t5068: F, t5075: F, t11883: F, t3507: F, t1755: F, t11871: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14986 = t14985 * t1246;
    let t14988 = t1235 * t5011;
    let t14989 = t14988 * t1246;
    let t14992 = t5072 * t5079;
    let t14997 = t5075 * t5068;
    let t15000 = t11883 * t3507;
    let t15001 = t1755 * t15000;
    let t15004 = t5072 * t5068;
    let t15009 = t1755 * t11871;
    (t14986, t14988, t14989, t14992, t14997, t15000, t15001, t15004, t15009)
}
