//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1440/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1440<F: Float>(t19883: F, t7923: F, t2201: F, t7290: F, t785: F, t788: F, t25573: F, t538: F, t7623: F, t2214: F, t514: F, t7300: F, t22796: F, t8139: F, t22829: F, t22843: F, t22857: F, t22861: F, t22863: F, t22865: F, t22871: F, t22873: F, t22883: F) -> (F,) {
    let t27084 = t19883 * t7923;
    let t27092 = t2201 * t785 * t788 * t7290;
    let t27095 = t7623 * t538 * t25573;
    let t27100 = t514 * t2214 * t7300;
    let t27102 = t22796 * t8139;
    let t27107 = 0.34930954652346593433e-1 * t27084 + 0.15256070262495512671e2 * t22829 + 0.69345773920434148505e1 * t22843 + 0.1536604809351619373e1 * t22857 + 0.25426783770825854452e1 * t22861 - 0.17465477326173296717e-1 * t27092 + 0.16463622957338778996e-1 * t27095 - 0.1047298617893752044e1 * t22863 + 0.17563392970889009433e0 * t22865 - 0.29272321618148349056e-1 * t27100 + 0.69861909304693186866e-1 * t27102 - 0.20958572791407956061e0 * t22871 - 0.17465477326173296717e-1 * t22873 - 0.9878173774403267398e-1 * t22883;
    (t27107,)
}
