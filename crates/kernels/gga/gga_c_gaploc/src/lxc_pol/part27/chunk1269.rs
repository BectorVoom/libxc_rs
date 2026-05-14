//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1269/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1269<F: Float>(t27868: F, t32300: F, t32302: F, t32304: F, t32307: F, t32308: F, t32309: F, t32310: F, t32311: F, t32312: F, t739: F, t12167: F, t12172: F, t12173: F, t12210: F, t12213: F, t12259: F, t1445: F, t1628: F, t1880: F, t2049: F, t2159: F, t2194: F, t28729: F, t28731: F, t33590: F, t3733: F, t3741: F, t3745: F, t3746: F, t39166: F, t4598: F, t4614: F, t531: F, t5629: F, t568: F, t5715: F, t6021: F, t6024: F, t797: F, t807: F, t813: F, t833: F, t836: F) -> (F, F, F) {
    let t39181 = t32300 - t32302 + t32304 + t27868 + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t39188 = t739 * t39181;
    let t39208 = t28729 + t28731 - 0.61348681526273199482e1 * t813 * t1628 * t12172 + 0.1022478025437886658e1 * t833 * t4598 * t3745 - 0.79445533226334281487e-1 * t3733 * t2159 - 0.46011511144704899612e1 * t2194 * t12173 + 0.23005755572352449806e1 * t833 * t568 * t836 * t39181 - 0.71500979903700853338e0 * t2049 * t12167 - 0.35750489951850426669e0 * t797 * t531 * t39188 - 0.23005755572352449806e1 * t6021 * t3741 + 0.23005755572352449806e1 * t6024 * t3746 - t33590 + 0.46011511144704899612e1 * t5629 * t1445 * t12213 * t1880 - 0.47667319935800568892e0 * t12210 * t5715 + 0.61348681526273199482e1 * t807 * t4614 * t12259 + 0.23005755572352449806e1 * t807 * t1445 * t39166;
    (t39181, t39188, t39208)
}
