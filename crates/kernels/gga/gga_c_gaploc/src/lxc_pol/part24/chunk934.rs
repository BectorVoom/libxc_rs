//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 934/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk934<F: Float>(t10820: F, t10931: F, t10930: F, t9982: F, t2676: F, t8775: F, t2536: F, t3038: F, t787: F, t2028: F, t2679: F, t3005: F, t9800: F, t10905: F, t10908: F, t10911: F, t10918: F, t10921: F, t10923: F, t10927: F, t9935: F, t9937: F, t9942: F, t9946: F) -> (F, F, F, F, F) {
    let t10932 = t10931 * t10820;
    let t10934 = 0.27606906686822939767e2 * t10930 * t10932;
    let t10935 = 0.63904876589867916128e-1 * t9982;
    let t10937 = 0.11916829983950142223e0 * t8775 * t2676;
    let t10938 = t2536 * t3038;
    let t10939 = t787 * t10938;
    let t10941 = 0.39722766613167140743e-1 * t10939 * t2028;
    let t10942 = t3005 * t2679;
    let t10943 = t9800 * t10942;
    let t10944 = 0.9585731488480187419e0 * t10943;
    let t10945 = t10905 - t10908 + t10911 - t10918 + t10921 + t10923 - t10927 + t10934 + t9935 + t9937 - t9942 - t9946 + t10935 + t10937 - t10941 + t10944;
    (t10932, t10938, t10939, t10942, t10945)
}
