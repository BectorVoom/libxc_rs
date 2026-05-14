//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 751/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk751<F: Float>(t1775: F, t3520: F, t11717: F, t3510: F, t1985: F, t2075: F, t3518: F, t12839: F, t12840: F, t12843: F, t12846: F, t12850: F, t12852: F, t12855: F, t12860: F, t462: F, t9178: F, t9179: F, t9188: F, t9190: F, t9202: F, t9205: F, t9207: F, t9209: F, t9241: F) -> (F,) {
    let t12864 = 4.0 / 3.0 * t1775 * t3520;
    let t12865 = t11717 * t3510;
    let t12868 = t1985 * t3518 * t2075;
    let t12878 = t12839 + 2.0 / 3.0 * t462 * t12840 + t462 * t12843 / 3.0 + 2.0 / 9.0 * t462 * t12846 - t12850 - 8.0 / 9.0 * t9179 - 4.0 / 27.0 * t12852 - 6.0 * t462 * t12855 + 4.0 * t462 * t12860 - t12864 + 22.0 / 9.0 * t12865 + 2.0 * t462 * t12868 - t9178 - 2.0 / 3.0 * t9209 + t9241 / 3.0 - 2.0 / 9.0 * t9190 - 8.0 / 27.0 * t9202 + t9205 / 9.0 + 2.0 / 27.0 * t9207 - 2.0 / 9.0 * t9188;
    (t12878,)
}
