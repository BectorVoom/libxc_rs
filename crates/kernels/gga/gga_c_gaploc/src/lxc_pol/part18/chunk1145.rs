//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1145/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1145<F: Float>(t25198: F, t7391: F, t3487: F, t739: F, t7803: F, t7805: F, t10939: F, t5694: F, t2617: F, t2963: F, t10834: F, t22883: F, t10667: F, t10721: F, t10996: F, t11075: F, t11096: F, t11113: F, t1445: F, t1890: F, t1966: F, t2004: F, t2033: F, t2049: F, t2194: F, t28443: F, t28450: F, t28454: F, t32186: F, t32504: F, t4673: F, t549: F, t5577: F, t5715: F, t590: F, t813: F) -> (F,) {
    let t33178 = t25198 * t7391;
    let t33179 = 0.89376224879626066674e-1 * t33178;
    let t33182 = t7803 * t739 * t3487 * t7805;
    let t33183 = 0.76685851907841499352e0 * t33182;
    let t33187 = 0.92686455430723328401e-1 * t10939 * t5694;
    let t33193 = t7803 * t2963 * t2617;
    let t33194 = 0.38342925953920749676e0 * t33193;
    let t33195 = t22883 * t10834;
    let t33196 = 0.29792074959875355558e-1 * t33195;
    let t33200 = -t28443 - 0.92023022289409799224e1 * t813 * t1445 * t32186 + 0.47667319935800568892e0 * t2004 * t4673 * t10721 - 0.1022478025437886658e1 * t5577 * t11113 - 0.1022478025437886658e1 * t1966 * t1890 * t10667 * t590 + t33179 + t33183 - 0.47667319935800568892e0 * t10996 * t5715 + t33187 - 0.47667319935800568892e0 * t2049 * t11075 - 0.46011511144704899612e1 * t2194 * t11096 + t28450 + t33194 + t28454 + t33196 + 0.79445533226334281486e-1 * t2033 * t549 * t32504;
    (t33200,)
}
