//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1162/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1162<F: Float>(t27937: F, t3038: F, t27494: F, t3074: F, t8009: F, t9856: F, t8219: F, t9860: F, t22823: F, t9853: F, t9864: F, t22727: F, t9868: F, t11227: F, t11302: F, t11305: F, t1208: F, t1209: F, t2296: F, t2318: F, t27795: F, t3135: F, t3136: F, t3807: F, t3819: F, t6266: F, t6300: F, t6323: F, t8071: F, t889: F, t9878: F, t9929: F) -> (F, F, F, F, F, F, F, F) {
    let t31327 = 6.0 * t27937 * t3038;
    let t31329 = 0.48245938496077605201e2 * t27494 * t3074;
    let t31331 = 6.0 * t8009 * t9856;
    let t31333 = 0.48245938496077605201e2 * t8219 * t9860;
    let t31335 = 0.2894756309764656312e3 * t22823 * t9853;
    let t31337 = 0.96491876992155210402e2 * t8219 * t9864;
    let t31339 = 0.1551780387578202009e4 * t22727 * t9868;
    let t31345 = -0.70178683471615754484e1 * t8071 * t9878 - 0.14035736694323150897e2 * t6323 * t11227 * t889 + 0.10526802520742363173e2 * t2318 * t3807 * t3135 - 0.35089341735807877242e1 * t6266 * t11302 - 0.35089341735807877242e1 * t2296 * t3136 * t3819 - 0.35089341735807877242e1 * t2296 * t1209 * t9929 + t31327 - t31329 + t31331 - t31333 + t31335 - t31337 - t31339 + 0.51947577317044391277e2 * t6300 * t11305 + 0.51947577317044391277e2 * t2318 * t27795 * t1208;
    (t31327, t31329, t31331, t31333, t31335, t31337, t31339, t31345)
}
