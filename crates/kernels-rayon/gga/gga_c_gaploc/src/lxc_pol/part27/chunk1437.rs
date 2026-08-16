//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1437/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1437(t27868: f64, t32300: f64, t32302: f64, t32304: f64, t32307: f64, t32308: f64, t32309: f64, t32310: f64, t32311: f64, t32312: f64, t739: f64, t12167: f64, t12172: f64, t12173: f64, t12210: f64, t12213: f64, t12259: f64, t1445: f64, t1628: f64, t1880: f64, t2049: f64, t2159: f64, t2194: f64, t28729: f64, t28731: f64, t33590: f64, t3733: f64, t3741: f64, t3745: f64, t3746: f64, t39166: f64, t4598: f64, t4614: f64, t531: f64, t5629: f64, t568: f64, t5715: f64, t6021: f64, t6024: f64, t797: f64, t807: f64, t813: f64, t833: f64, t836: f64) -> (f64, f64, f64) {
    let t39181 = t32300 - t32302 + t32304 + t27868 + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t39188 = t739 * t39181;
    let t39208 = t28729 + t28731 - 0.61348681526273199482e1_f64 * t813 * t1628 * t12172 + 0.1022478025437886658e1_f64 * t833 * t4598 * t3745 - 0.79445533226334281487e-1_f64 * t3733 * t2159 - 0.46011511144704899612e1_f64 * t2194 * t12173 + 0.23005755572352449806e1_f64 * t833 * t568 * t836 * t39181 - 0.71500979903700853338e0_f64 * t2049 * t12167 - 0.35750489951850426669e0_f64 * t797 * t531 * t39188 - 0.23005755572352449806e1_f64 * t6021 * t3741 + 0.23005755572352449806e1_f64 * t6024 * t3746 - t33590 + 0.46011511144704899612e1_f64 * t5629 * t1445 * t12213 * t1880 - 0.47667319935800568892e0_f64 * t12210 * t5715 + 0.61348681526273199482e1_f64 * t807 * t4614 * t12259 + 0.23005755572352449806e1_f64 * t807 * t1445 * t39166;
    (t39181, t39188, t39208)
}
